// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure-specific tracing conventions and constants.

/// Azure service name attribute key
pub const AZURE_SERVICE_NAME: &str = "azure.service.name";

/// Azure operation name attribute key
pub const AZURE_OPERATION_NAME: &str = "azure.operation.name";

/// Azure request ID attribute key
pub const AZURE_REQUEST_ID: &str = "azure.request.id";

/// Azure client name attribute key
pub const AZURE_CLIENT_NAME: &str = "azure.client.name";

/// Azure client version attribute key
pub const AZURE_CLIENT_VERSION: &str = "azure.client.version";

/// Azure namespace attribute key
pub const AZURE_NAMESPACE: &str = "azure.namespace";

/// Azure resource provider namespace attribute key
pub const AZURE_RESOURCE_PROVIDER_NAMESPACE: &str = "azure.resource_provider.namespace";
