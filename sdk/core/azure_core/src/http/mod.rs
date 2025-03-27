// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

pub mod headers;
mod models;
mod options;
mod pager;
mod pipeline;
pub mod policies;
pub mod poller;
pub mod request;

pub use models::*;
pub use options::*;
pub use pager::*;
pub use pipeline::*;
pub use request::{Body, Request, RequestContent};
pub use response::{Model, Response};

pub use typespec_client_core::http::response;
pub use typespec_client_core::http::{
    new_http_client, AppendToUrlQuery, Context, HttpClient, Method, StatusCode, Url,
};
