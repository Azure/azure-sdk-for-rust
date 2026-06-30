// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Integration tests legitimately compose many Cosmos operation futures (each
// near clippy's default 16 KiB threshold) and run on tokio's large-stack test
// threads, so the production-oriented `large_futures` lint is allowed here.
#![allow(clippy::large_futures)]
mod gateway_v2_tests;
