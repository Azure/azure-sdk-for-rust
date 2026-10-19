// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Offline proxy checks with process-isolated environment variables and loopback listeners.

use azure_core::credentials::Secret;
use azure_data_cosmos::{
    options::{ConnectionPoolOptions, Region},
    AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use std::{
    error::Error,
    io::{ErrorKind, Read},
    net::TcpListener,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

const CHILD_TEST: &str = "COSMOS_PROXY_TEST_CHILD";
const TARGET_ENDPOINT: &str = "COSMOS_PROXY_TEST_ENDPOINT";

#[derive(Clone, Copy)]
enum ProxyMode {
    Default,
    Enabled,
    Bypassed,
}

fn run_proxy_test(name: &str, mode: ProxyMode) -> Result<(), Box<dyn Error>> {
    if std::env::var(CHILD_TEST).as_deref() == Ok(name) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        return runtime.block_on(async {
            let mut pool = ConnectionPoolOptions::builder()
                .with_max_connect_timeout(Duration::from_secs(1))
                .with_max_metadata_request_timeout(Duration::from_secs(1));
            if !matches!(mode, ProxyMode::Default) {
                pool = pool.with_proxy_allowed(true);
            }
            let runtime = CosmosRuntime::builder()
                .with_connection_pool(pool.build()?)
                .build()
                .await?;
            let endpoint = std::env::var(TARGET_ENDPOINT)?.parse()?;
            let account = AccountReference::with_authentication_key(endpoint, Secret::from("a2V5"));
            let result = tokio::time::timeout(
                Duration::from_secs(10),
                CosmosClient::builder()
                    .with_runtime(runtime)
                    .build(account, RoutingStrategy::ProximityTo(Region::EAST_US)),
            )
            .await?;
            assert!(
                result.is_err(),
                "the fake listeners never serve account metadata"
            );
            Ok(())
        });
    }

    let target = TcpListener::bind("127.0.0.1:0")?;
    let proxy = TcpListener::bind("127.0.0.1:0")?;
    target.set_nonblocking(true)?;
    proxy.set_nonblocking(true)?;
    let authority = target.local_addr()?.to_string();
    let proxy_url = format!("http://{}", proxy.local_addr()?);
    let mut command = Command::new(std::env::current_exe()?);
    command
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env(CHILD_TEST, name)
        .env(TARGET_ENDPOINT, format!("https://{authority}/"))
        .env("COSMOS_DISABLE_IMDS", "true")
        .env("HTTP_PROXY", &proxy_url)
        .env("HTTPS_PROXY", &proxy_url)
        .env("http_proxy", &proxy_url)
        .env("https_proxy", &proxy_url)
        .env_remove("ALL_PROXY")
        .env_remove("all_proxy")
        .env_remove("NO_PROXY")
        .env_remove("no_proxy")
        .env_remove("REQUEST_METHOD")
        .env_remove("AZURE_COSMOS_CONNECTION_POOL_IS_PROXY_ALLOWED")
        .env("RUST_LOG", "off")
        .stdin(Stdio::null());
    if matches!(mode, ProxyMode::Bypassed) {
        command.env("NO_PROXY", "127.0.0.1");
    }
    let mut child = command.spawn()?;
    let result = (|| {
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut proxy_connections = 0_u32;
        let mut target_connections = 0_u32;
        loop {
            match proxy.accept() {
                Ok((mut stream, _)) => {
                    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
                    let mut first_line = Vec::new();
                    loop {
                        let mut byte = [0_u8];
                        if stream.read(&mut byte)? == 0 {
                            return Err("proxy connection closed before its request line".into());
                        }
                        first_line.push(byte[0]);
                        if byte[0] == b'\n' {
                            break;
                        }
                        if first_line.len() >= 4096 {
                            return Err("proxy request line exceeded its bound".into());
                        }
                    }
                    let expected = format!("CONNECT {authority} HTTP/1.1\r\n");
                    if first_line != expected.as_bytes() {
                        return Err("proxy received traffic for a target other than Cosmos".into());
                    }
                    proxy_connections += 1;
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {}
                Err(error) => return Err(error.into()),
            }
            match target.accept() {
                Ok((_stream, _)) => target_connections += 1,
                Err(error) if error.kind() == ErrorKind::WouldBlock => {}
                Err(error) => return Err(error.into()),
            }
            if let Some(status) = child.try_wait()? {
                if !status.success() {
                    return Err(format!("proxy test child failed: {status}").into());
                }
                let expected_proxy = matches!(mode, ProxyMode::Enabled);
                if (proxy_connections > 0) != expected_proxy
                    || (target_connections > 0) == expected_proxy
                {
                    return Err(format!(
                        "unexpected route: proxy={proxy_connections}, target={target_connections}"
                    )
                    .into());
                }
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err("proxy test child exceeded its deadline".into());
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    })();
    if child.try_wait()?.is_none() {
        child.kill()?;
    }
    child.wait()?;
    result
}

#[test]
fn proxy_disabled_by_default_ignores_env() -> Result<(), Box<dyn Error>> {
    run_proxy_test(
        "emulator_tests::cosmos_proxy::proxy_disabled_by_default_ignores_env",
        ProxyMode::Default,
    )
}

#[test]
fn proxy_enabled_routes_through_proxy() -> Result<(), Box<dyn Error>> {
    run_proxy_test(
        "emulator_tests::cosmos_proxy::proxy_enabled_routes_through_proxy",
        ProxyMode::Enabled,
    )
}

#[test]
fn proxy_exclusion_routes_directly() -> Result<(), Box<dyn Error>> {
    run_proxy_test(
        "emulator_tests::cosmos_proxy::proxy_exclusion_routes_directly",
        ProxyMode::Bypassed,
    )
}
