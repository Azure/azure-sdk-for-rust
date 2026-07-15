// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod config;
mod data_plane;
mod gateway_v2;
mod management;
mod metrics;

use std::{
    io::{self, Write},
    net::SocketAddr,
    path::PathBuf,
};

use clap::Parser;
use config::{EmulatorConfig, GatewayBinding};
use metrics::HostMetrics;
use serde::Serialize;
use tokio::net::TcpListener;
use url::Url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Parser)]
#[command(about = "Hosts the Azure Cosmos DB in-memory emulator over HTTP")]
struct Args {
    /// JSON configuration file describing the virtual account and seed data.
    #[arg(long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "azure_data_cosmos_emulator=info".into()),
        )
        .init();

    let args = Args::parse();
    let config = EmulatorConfig::load(&args.config).await?;
    let bound_gateways = config.bind_gateways().await?;
    let bindings: Vec<_> = bound_gateways
        .iter()
        .map(|gateway| gateway.binding.clone())
        .collect();
    let management_listener =
        TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.management.port))).await?;
    let management_endpoint = loopback_url(management_listener.local_addr()?.port())?;
    let account_endpoint = bindings
        .first()
        .ok_or("no emulator gateway listeners were configured")?
        .gateway_url
        .clone();
    let emulator = config.create_emulator(&bindings)?;
    config.provision(&emulator, &account_endpoint).await?;
    let metrics = std::sync::Arc::new(HostMetrics::default());

    let mut listeners = tokio::task::JoinSet::new();
    for bound_gateway in bound_gateways {
        let binding = bound_gateway.binding;
        let gateway_emulator = emulator.clone();
        let gateway_binding = binding.clone();
        listeners.spawn(async move {
            data_plane::serve(
                bound_gateway.gateway_listener,
                gateway_binding,
                gateway_emulator,
            )
            .await
        });
        if let Some(gateway20_listener) = bound_gateway.gateway20_listener {
            let gateway20_emulator = emulator.clone();
            let metrics = metrics.clone();
            listeners.spawn(async move {
                gateway_v2::serve(gateway20_listener, binding, gateway20_emulator, metrics).await
            });
        }
    }
    let account_id = config.account.id.clone();
    let management_bindings = bindings.clone();
    listeners.spawn(async move {
        management::serve(
            management_listener,
            emulator,
            account_id,
            management_bindings,
            metrics,
        )
        .await
    });
    write_ready_record(management_endpoint, account_endpoint, &bindings)?;

    match listeners.join_next().await {
        Some(Ok(Ok(()))) => Err("an emulator listener stopped unexpectedly".into()),
        Some(Ok(Err(error))) => Err(error.into()),
        Some(Err(error)) => Err(error.into()),
        None => Err("no emulator listeners were configured".into()),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadyRecord {
    event: &'static str,
    management_endpoint: String,
    account_endpoint: String,
    regions: Vec<ReadyRegion>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadyRegion {
    name: String,
    gateway_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway20_endpoint: Option<String>,
}

fn write_ready_record(
    management_endpoint: Url,
    account_endpoint: Url,
    bindings: &[GatewayBinding],
) -> Result<()> {
    let record = ReadyRecord {
        event: "ready",
        management_endpoint: management_endpoint.into(),
        account_endpoint: account_endpoint.into(),
        regions: bindings
            .iter()
            .map(|binding| ReadyRegion {
                name: binding.region_name.clone(),
                gateway_endpoint: binding.gateway_url.as_str().to_owned(),
                gateway20_endpoint: binding
                    .gateway20_url
                    .as_ref()
                    .map(|url| url.as_str().to_owned()),
            })
            .collect(),
    };
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    serde_json::to_writer(&mut stdout, &record)?;
    writeln!(stdout)?;
    stdout.flush()?;
    Ok(())
}

fn loopback_url(port: u16) -> Result<Url> {
    Ok(Url::parse(&format!("http://127.0.0.1:{port}/"))?)
}
