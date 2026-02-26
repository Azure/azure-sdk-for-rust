// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CLI argument parsing and configuration.

use clap::Parser;

/// Performance and scale testing tool for the Azure Cosmos DB Rust SDK.
#[derive(Parser, Debug)]
#[command(name = "azure_data_cosmos_perf")]
pub struct Config {
    /// Cosmos DB account endpoint URL.
    #[arg(long)]
    pub endpoint: String,

    /// Database name.
    #[arg(long, default_value = "perfdb")]
    pub database: String,

    /// Container name.
    #[arg(long, default_value = "perfcontainer")]
    pub container: String,

    /// Authentication method.
    #[arg(long, value_enum)]
    pub auth: AuthMethod,

    /// Account key (required when --auth=key). Can also be set via AZURE_COSMOS_KEY env var.
    #[arg(long, env = "AZURE_COSMOS_KEY")]
    pub key: Option<String>,

    /// Comma-separated list of preferred regions.
    #[arg(long, value_delimiter = ',')]
    pub preferred_regions: Vec<String>,

    /// Comma-separated list of regions to exclude from routing.
    ///
    /// When set, the specified regions are skipped for the operations
    /// indicated by `--exclude-regions-for`. Region names must match
    /// Azure region display names (e.g. "West US", "East US 2").
    #[arg(long, value_delimiter = ',')]
    pub excluded_regions: Vec<String>,

    /// Which operation types the excluded regions apply to.
    #[arg(long, value_enum, default_value_t = ExcludeRegionsScope::Both)]
    pub exclude_regions_for: ExcludeRegionsScope,

    /// Disable point read operations.
    #[arg(long, default_value_t = false)]
    pub no_reads: bool,

    /// Disable query operations.
    #[arg(long, default_value_t = false)]
    pub no_queries: bool,

    /// Disable upsert operations.
    #[arg(long, default_value_t = false)]
    pub no_upserts: bool,

    /// Disable create operations.
    #[arg(long, default_value_t = false)]
    pub no_creates: bool,

    /// Number of concurrent operations (minimum: 1).
    #[arg(long, default_value_t = 50)]
    pub concurrency: usize,

    /// Run duration in seconds. Omit for indefinite.
    #[arg(long)]
    pub duration: Option<u64>,

    /// Number of items to seed into the container (minimum: 1).
    #[arg(long, default_value_t = 1000)]
    pub seed_count: usize,

    /// Stats reporting interval in seconds.
    #[arg(long, default_value_t = 300)]
    pub report_interval: u64,

    /// Throughput (RU/s) to provision when creating the container.
    #[arg(long, default_value_t = 100000)]
    pub throughput: usize,

    /// Container name for storing perf results as JSON documents.
    ///
    /// Each reporting interval's metrics are upserted into this container
    /// for long-term monitoring and Kusto/dashboard integration.
    /// The container is auto-created if it does not exist.
    #[arg(long, default_value = "perfresults")]
    pub results_container: String,

    /// Cosmos DB endpoint for the results account.
    ///
    /// When set, results are written to a separate Cosmos DB account so they
    /// don't add load to the account being tested. When omitted, results go
    /// to the same account as `--endpoint`.
    #[arg(long)]
    pub results_endpoint: Option<String>,

    /// Database name on the results account.
    #[arg(long, default_value = "perfdb")]
    pub results_database: String,

    /// Authentication method for the results account.
    ///
    /// Defaults to the same method as `--auth` when omitted.
    #[arg(long, value_enum)]
    pub results_auth: Option<AuthMethod>,

    /// Account key for the results account (required when --results-auth=key).
    /// Can also be set via AZURE_COSMOS_RESULTS_KEY env var.
    #[arg(long, env = "AZURE_COSMOS_RESULTS_KEY")]
    pub results_key: Option<String>,

    /// Default time-to-live in seconds for items in the container.
    ///
    /// Applied when creating new containers. Set to 0 to disable TTL.
    /// Items expire after this duration unless overridden per-item.
    #[arg(long, default_value_t = 3600)]
    pub default_ttl: u64,

    /// Unique identifier for this workload instance.
    ///
    /// Stamped on every result and error document so runs from different
    /// VMs or configurations can be distinguished when sharing the same
    /// results container. Defaults to a random UUID.
    #[arg(long, default_value_t = uuid::Uuid::new_v4().to_string())]
    pub workload_id: String,
}

/// Authentication method for connecting to Cosmos DB.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AuthMethod {
    /// Key-based authentication using a primary or secondary account key.
    Key,
    /// Microsoft Entra ID (AAD) authentication using ManagedIdentityCredential (managed identity).
    Aad,
}

/// Selects which operation types excluded regions apply to.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum ExcludeRegionsScope {
    /// Exclude regions only for read operations (point reads).
    Reads,
    /// Exclude regions only for write operations (creates, upserts).
    Writes,
    /// Exclude regions for both reads and writes.
    Both,
}
