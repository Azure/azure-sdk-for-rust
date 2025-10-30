// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options sent with requests to the service.

#[macro_use]
mod macros;

use crate::http::headers::CONTENT_TYPE;

request_header!(
    #[doc = "The Content Type indicates the media type of the request body"]
    ContentType,
    CONTENT_TYPE,
    (
        #[doc = "The content type for JSON payloads"]
        APPLICATION_JSON,
        "application/json"
    )
);
