// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Storage Queue service.

mod queue_client;
pub use queue_client::{QueueClient, QueueClientOptions};

mod queue_service_client;
pub use queue_service_client::{QueueServiceClient, QueueServiceClientOptions};
