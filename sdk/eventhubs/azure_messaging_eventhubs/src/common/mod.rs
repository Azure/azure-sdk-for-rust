// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub(crate) mod management;
pub(crate) mod recoverable_connection;
pub mod retry;
pub(crate) mod user_agent;

// Public API
pub(crate) use retry::{retry_azure_operation, RetryOptions};

pub(crate) use management::ManagementInstance;
