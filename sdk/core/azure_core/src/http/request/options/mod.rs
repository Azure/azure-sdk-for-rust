// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options sent with requests to the service.

use crate::http::headers::CLIENT_REQUEST_ID;
pub use typespec_client_core::http::request::options::*;
use typespec_client_core::request_header;

request_header!(
    /// The `x-ms-client-request-id` header.
    ///
    /// # Examples
    ///
    /// Add a caller-defined client request ID to a request.
    ///
    /// ```
    /// use azure_core::{
    ///     http::{ClientMethodOptions, request::options::ClientRequestId},
    ///     Uuid,
    /// };
    /// let client_request_id: String = Uuid::new_v4().into();
    /// let mut options = ClientMethodOptions::default();
    /// options.context.insert(ClientRequestId::new(client_request_id));
    /// ```
    ClientRequestId,
    CLIENT_REQUEST_ID,
);
