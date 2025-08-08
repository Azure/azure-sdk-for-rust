// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Common functionality for Service Bus.

/// Authorization functionality for Service Bus operations.
///
/// Handles Entra ID authentication tokens and automatic token refresh for Service Bus resources.
pub mod authorizer;

/// Retry functionality for Service Bus operations.
pub mod retry;
