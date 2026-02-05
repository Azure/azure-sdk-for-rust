// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ARM-specific HTTP pipeline policies.
//!
//! This module provides policies specific to Azure Resource Manager (ARM) operations,
//! including automatic resource provider registration and ARM-tuned retry configurations.
//!
//! # Resource Provider Registration
//!
//! The [`RPRegistrationPolicy`] automatically registers unregistered resource providers
//! when encountering registration errors. This is particularly useful when working with
//! Azure Resource Manager as it eliminates the need for manual provider registration.
//!
//! ## Example: Using RPRegistrationPolicy
//!
//! ```no_run
//! use azure_core::{
//!     resourcemanager::{RPRegistrationOptions, RPRegistrationPolicy},
//!     credentials::TokenCredential,
//!     http::{ClientOptions, Pipeline},
//! };
//! use std::sync::Arc;
//!
//! # async fn example(credential: Arc<dyn TokenCredential>) -> azure_core::Result<()> {
//! // Create RP registration policy with default options
//! let rp_policy = RPRegistrationPolicy::new(
//!     credential.clone(),
//!     RPRegistrationOptions::default(),
//! );
//!
//! // Add to pipeline as a per-try policy
//! let mut client_options = ClientOptions::default();
//! client_options.per_try_policies.push(Arc::new(rp_policy));
//!
//! // Create pipeline with the policy
//! let pipeline = Pipeline::new(
//!     Some("my-service"),
//!     Some("1.0.0"),
//!     client_options,
//!     Vec::new(),
//!     Vec::new(),
//!     None,
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # ARM-Specific Retry Configuration
//!
//! ARM operations often require different retry configurations than data plane operations.
//! This module provides ARM-tuned retry options with appropriate defaults.
//!
//! ## Example: Configuring ARM Retry Behavior
//!
//! ```rust
//! use azure_core::{
//!     resourcemanager::{arm_exponential_retry_options, arm_exponential_retry_options_with, ArmExponentialRetryOptions},
//!     http::ClientOptions,
//!     time::Duration,
//! };
//!
//! // Use default ARM retry configuration
//! let mut client_options = ClientOptions::default();
//! arm_exponential_retry_options(&mut client_options);
//!
//! // Or customize the retry behavior
//! let mut custom_options = ClientOptions::default();
//! let custom_retry = ArmExponentialRetryOptions {
//!     initial_delay: Duration::seconds(2),
//!     max_retries: 5,
//!     max_total_elapsed: Duration::minutes(5),
//!     max_delay: Duration::seconds(30),
//! };
//! arm_exponential_retry_options_with(&mut custom_options, custom_retry);
//! ```
//!
//! # Complete ARM Client Example
//!
//! Here's a complete example showing how to create an ARM client with both
//! RP registration and custom retry configuration:
//!
//! ```no_run
//! use azure_core::{
//!     resourcemanager::{arm_exponential_retry_options, RPRegistrationOptions, RPRegistrationPolicy},
//!     credentials::TokenCredential,
//!     http::{ClientOptions, Pipeline},
//! };
//! use std::sync::Arc;
//!
//! # async fn create_arm_pipeline(credential: Arc<dyn TokenCredential>) -> azure_core::Result<Pipeline> {
//! // Configure retry behavior for ARM
//! let mut client_options = ClientOptions::default();
//! arm_exponential_retry_options(&mut client_options);
//!
//! // Create and add RP registration policy
//! let rp_policy = RPRegistrationPolicy::new(
//!     credential,
//!     RPRegistrationOptions::default(),
//! );
//! client_options.per_try_policies.push(Arc::new(rp_policy));
//!
//! // Create the pipeline
//! let pipeline = Pipeline::new(
//!     Some("my-arm-client"),
//!     Some("1.0.0"),
//!     client_options,
//!     Vec::new(),
//!     Vec::new(),
//!     None,
//! );
//!
//! Ok(pipeline)
//! # }
//! ```

mod retry;
mod rp_registration;

pub use retry::*;
pub use rp_registration::*;
