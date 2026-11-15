// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Compile-time and usage contracts for the change feed processor.
//!
//! ## Explicit callback type
//!
//! ```
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedHandler,
//!     ChangeFeedProcessor, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, ContainerClient,
//! };
//! use std::future::{ready, Ready};
//!
//! fn on_changes(
//!     _: ChangeFeedProcessorContext,
//!     _: Vec<ChangeFeedItem<String>>,
//! ) -> Ready<azure_data_cosmos::Result<()>> {
//!     ready(Ok(()))
//! }
//!
//! fn configure(source: ContainerClient, leases: ContainerClient) -> azure_data_cosmos::Result<()> {
//!     let handler: ChangeFeedHandler<String, Ready<azure_data_cosmos::Result<()>>> = on_changes;
//!     let builder: ChangeFeedProcessorBuilder<String, Ready<azure_data_cosmos::Result<()>>> =
//!         ChangeFeedProcessorBuilder::new(
//!             ChangeFeedMode::LatestVersion, source, leases, "orders", "worker", handler,
//!         );
//!     let _processor: ChangeFeedProcessor<String, Ready<azure_data_cosmos::Result<()>>> =
//!         builder.with_start_from_beginning().build()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Async callbacks in both feed modes
//!
//! ```
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder,
//!     ChangeFeedProcessorContext, ContainerClient,
//! };
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Order { id: String }
//!
//! async fn on_changes(
//!     context: ChangeFeedProcessorContext,
//!     changes: Vec<ChangeFeedItem<Order>>,
//! ) -> azure_data_cosmos::Result<()> {
//!     println!("Lease: {}", context.lease_token());
//!     for change in changes {
//!         if let Some(order) = change.current() {
//!             println!("Changed order: {}", order.id);
//!         }
//!     }
//!     Ok(())
//! }
//!
//! fn configure(source: ContainerClient, leases: ContainerClient) -> azure_data_cosmos::Result<()> {
//!     for mode in [ChangeFeedMode::LatestVersion, ChangeFeedMode::AllVersionsAndDeletes] {
//!         let _processor = ChangeFeedProcessorBuilder::new(
//!             mode, source.clone(), leases.clone(), "orders", "worker", on_changes,
//!         ).build()?;
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration helpers preserve the callback type
//!
//! ```
//! use azure_data_cosmos::{
//!     feed::ContinuationToken, options::MaxItemCountHint,
//!     ChangeFeedCheckpointPolicy, ChangeFeedProcessorBuilder,
//! };
//! use std::{num::NonZeroU32, time::Duration};
//!
//! fn configure<T, Fut>(
//!     builder: ChangeFeedProcessorBuilder<T, Fut>,
//!     continuation: ContinuationToken,
//! ) -> ChangeFeedProcessorBuilder<T, Fut> {
//!     builder
//!         .with_checkpoint_policy(ChangeFeedCheckpointPolicy::Deferred {
//!             max_interval: Duration::from_secs(30),
//!         })
//!         .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(200).unwrap()))
//!         .with_feed_poll_delay(Duration::from_secs(2))
//!         .with_bootstrap_timeout(Duration::from_secs(600))
//!         .with_start_continuation(continuation)
//!         .with_min_scale_count(1)
//!         .with_max_scale_count(4)
//!         .with_lease_verification_enabled_on_restart(true)
//! }
//! ```
//!
//! ## Monitoring signature
//!
//! ```
//! use azure_data_cosmos::ChangeFeedProcessor;
//!
//! async fn inspect<T, Fut>(processor: &ChangeFeedProcessor<T, Fut>) -> azure_data_cosmos::Result<()> {
//!     println!("Running: {}", processor.is_started());
//!     for lease in processor.current_state().await? {
//!         println!("{} {:?} {:?}", lease.lease_token(), lease.owner_worker_name(), lease.estimated_lag());
//!         let _saved_progress = lease.continuation_token();
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Construction requires clients, names, and a callback
//!
//! ```compile_fail,E0061
//! use azure_data_cosmos::{options::ChangeFeedMode, ChangeFeedProcessorBuilder};
//! let _builder = ChangeFeedProcessorBuilder::new(ChangeFeedMode::LatestVersion);
//! ```
//!
//! ## Construction requires an explicit mode
//!
//! ```compile_fail,E0061
//! use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, ContainerClient};
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
//!     );
//! }
//! ```
//!
//! ## A callback is required during construction
//!
//! ```compile_fail,E0061
//! use azure_data_cosmos::{options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient};
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!     );
//! }
//! ```
//!
//! ## Returning a synchronous result is not accepted
//!
//! ```compile_fail,E0277
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<String>>| Ok::<(), azure_data_cosmos::CosmosError>(()),
//!     );
//! }
//! ```
//!
//! ## The callback future must return a result
//!
//! ```compile_fail,E0271
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<String>>| async {},
//!     );
//! }
//! ```
//!
//! ## Item types must support deserialization
//!
//! ```compile_fail,E0277
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! struct NotDeserializable;
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<NotDeserializable>>| async { Ok(()) },
//!     );
//! }
//! ```
//!
//! ## The callback must accept both context and changes
//!
//! ```compile_fail,E0593,E0308
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_: Vec<ChangeFeedItem<String>>| async { Ok(()) },
//!     );
//! }
//! ```
//!
//! ## The callback cannot be replaced after construction
//!
//! ```compile_fail,E0599
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
//!     ).handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) });
//! }
//! ```
//!
//! ## Capturing closures are unsupported
//!
//! ```compile_fail,E0308
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let label = String::from("orders");
//!     let _builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         move |_, _: Vec<ChangeFeedItem<String>>| {
//!             let label = label.clone();
//!             async move { println!("{label}"); Ok(()) }
//!         },
//!     );
//! }
//! ```
//!
//! ## Building consumes the builder
//!
//! ```compile_fail,E0382
//! use azure_data_cosmos::{
//!     models::ChangeFeedItem, options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
//! };
//! fn configure(source: ContainerClient, leases: ContainerClient) {
//!     let builder = ChangeFeedProcessorBuilder::new(
//!         ChangeFeedMode::LatestVersion, source, leases, "orders", "worker",
//!         |_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) },
//!     );
//!     let _processor = builder.build();
//!     let _another = builder.build();
//! }
//! ```
