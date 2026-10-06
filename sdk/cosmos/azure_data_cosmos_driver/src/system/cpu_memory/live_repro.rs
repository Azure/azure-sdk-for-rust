// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    driver::CosmosDriverRuntime,
    models::{AccountReference, ConnectionString, CosmosOperation, DatabaseReference},
    options::{DiagnosticsVerbosity, DriverOptions, OperationOptions},
};
use azure_core::http::StatusCode;
use std::{
    error::Error,
    time::{Duration, Instant},
};

#[tokio::test]
#[ignore = "requires an explicitly configured live account on Linux or Windows"]
async fn live_cpu_diagnostics_repro() -> Result<(), Box<dyn Error>> {
    if !cfg!(any(target_os = "linux", target_os = "windows")) {
        return Err("real CPU sampling requires Linux or Windows".into());
    }
    let connection: ConnectionString = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?.parse()?;
    let account = AccountReference::with_master_key(
        connection.account_endpoint().parse()?,
        connection.account_key().secret().to_owned(),
    );
    let runtime = CosmosDriverRuntime::builder().build().await?;
    let driver = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await?;
    let database = DatabaseReference::from_name(account, "diagnostics-cpu-probe-5394".to_owned());
    let started = Instant::now();
    let mut after_drop = None;
    loop {
        let result = driver
            .execute_operation(
                CosmosOperation::read_database(database.clone()),
                OperationOptions::default(),
            )
            .await;
        let diagnostics = match &result {
            Ok(Some(response)) => response.diagnostics(),
            Err(error) if error.status().status_code() == StatusCode::NotFound => error
                .diagnostics()
                .ok_or("missing live error diagnostics")?,
            Ok(None) => return Err("read_database must produce a response".into()),
            Err(error) => return Err(error.to_string().into()),
        };
        let json: serde_json::Value =
            serde_json::from_str(diagnostics.to_json_string(Some(DiagnosticsVerbosity::Detailed)))?;
        drop(diagnostics);
        drop(result);
        let after_drop = *after_drop.get_or_insert_with(Instant::now);
        let history = runtime.cpu_monitor().snapshot();
        let fresh = history
            .samples()
            .iter()
            .filter(|sample| sample.timestamp > after_drop && sample.cpu.is_some())
            .count();
        if fresh >= 3 {
            if json["system_usage"]["cpu"]["status"] != "available" {
                return Err("fresh CPU measurements missing from serialized diagnostics".into());
            }
            println!(
                "CPU_REPRO fresh_samples={fresh} {}",
                json["system_usage"]["cpu"]
            );
            return Ok(());
        }
        if started.elapsed() >= Duration::from_secs(30) {
            return Err(format!(
                "CPU sampling stalled while the runtime remained alive: fresh_samples={fresh}, cpu={}",
                json["system_usage"]["cpu"],
            ).into());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}
