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
    #[arg(long)]
    pub database: String,

    /// Container name.
    #[arg(long)]
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

    /// Comma-separated list of excluded regions.
    #[arg(long, value_delimiter = ',')]
    pub excluded_regions: Vec<String>,

    /// Disable point read operations.
    #[arg(long, default_value_t = false)]
    pub no_reads: bool,

    /// Disable query operations.
    #[arg(long, default_value_t = false)]
    pub no_queries: bool,

    /// Disable upsert operations.
    #[arg(long, default_value_t = false)]
    pub no_upserts: bool,

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
}

/// Authentication method for connecting to Cosmos DB.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AuthMethod {
    /// Key-based authentication using a primary or secondary account key.
    Key,
    /// Microsoft Entra ID (AAD) authentication using DeveloperToolsCredential.
    Aad,
}
