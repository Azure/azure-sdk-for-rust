// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod change_feed_poller;
mod cosmos_lease_store;
mod lease_load_balancer;
mod lease_processing;
mod lease_topology_handler;
mod managed_processor;
mod prepared_container;
mod processor_engine;

pub use change_feed_poller::{
    BatchState, ChangeFeedMode, ChangeFeedReadOptions, ChangeFeedReader, RawBatchSource,
    RawChangeFeedPage,
};
pub use cosmos_lease_store::{
    BalanceCycle, BalanceRunOptions, BalanceRunReport, BootstrapPlan, BootstrapReady,
    BootstrapStartPolicy, BootstrapStore, CosmosLeaseStore, InitialLease, LeaseBalancer,
    LeaseObservation, LeaseOwnershipOptions, LeaseOwnershipRun, LeaseReleaseOutcome, LeaseSession,
};
pub use lease_load_balancer::{
    plan_equal_lease_balance, BalanceAction, BalanceActionKind, BalanceLease,
};
pub use lease_processing::{
    run_lease, CheckpointError, CheckpointStore, LeaseControl, LeaseRunError, LeaseRunOptions,
    LeaseRunOutcome, LeaseRunPhase, LeaseRunReport, OwnedLease,
};
pub use lease_topology_handler::{plan_lease_topology, LeaseTopologyAssignment};
pub use managed_processor::{
    ManagedLeaseShutdown, ManagedLeaseSnapshot, ManagedLeaseState, ManagedProcessor,
    ManagedProcessorOptions, ManagedProcessorSnapshot, ManagedProcessorState,
    ManagedShutdownReport, RawChangeHandler,
};
pub use processor_engine::ProcessorEngine;

/// Compatibility name for [`ProcessorEngine`].
pub use processor_engine::ProcessorEngine as ChangeFeedProcessorEngine;
