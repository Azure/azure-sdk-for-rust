// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Runtime builder (Phase 2 — stub).
//!
//! Will mirror `azure_data_cosmos_driver::CosmosDriverRuntimeBuilder` so
//! callers can configure workload id, correlation id, user-agent suffix,
//! connection pool, and default operation options before
//! `cosmos_runtime_create`.

// TODO(phase-2): cosmos_runtime_builder_{new,free,build} + setters mirroring
// CosmosDriverRuntimeBuilder.
