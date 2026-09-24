// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{future::Future, panic::AssertUnwindSafe, sync::OnceLock, time::Duration};

use futures::FutureExt;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::e2e_test_cases::fixture::TestResult;

const REGIONS: [&str; 3] = ["East US", "West US", "North Europe"];

#[derive(Clone)]
pub(super) struct TopologyManager {
    endpoint: url::Url,
    client: reqwest::Client,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AccountTopology {
    pub(super) write_region: String,
    pub(super) regions: Vec<AccountRegion>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AccountRegion {
    pub(super) name: String,
    pub(super) writable: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ManagementOperation {
    pub(super) operation_id: String,
    pub(super) phase: String,
    #[serde(default)]
    pub(super) children: Vec<u32>,
    pub(super) into: Option<u32>,
    pub(super) error: Option<String>,
}

impl TopologyManager {
    pub(super) fn from_env() -> TestResult<Self> {
        let endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
        Ok(Self {
            endpoint: url::Url::parse(&endpoint)?,
            client: management_client().clone(),
        })
    }

    pub(super) async fn account(&self) -> TestResult<AccountTopology> {
        self.get_json(&["account"]).await
    }

    pub(super) async fn offline(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "offline"]).await
    }

    pub(super) async fn online(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "online"]).await
    }

    pub(super) async fn begin_removal(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "removal", "begin"])
            .await
    }

    pub(super) async fn cancel_removal(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "removal", "cancel"])
            .await
    }

    pub(super) async fn remove(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "remove"]).await
    }

    pub(super) async fn add(&self, region: &str) -> TestResult {
        self.post_json(&["regions", region, "add"], &json!({}))
            .await
    }

    pub(super) async fn announce_failover(&self, region: &str) -> TestResult {
        self.post_empty(&["failover", region, "announce"]).await
    }

    pub(super) async fn begin_failover(&self, region: &str) -> TestResult {
        self.post_empty(&["failover", region, "begin"]).await
    }

    pub(super) async fn complete_failover(&self) -> TestResult {
        self.post_empty(&["failover", "complete"]).await
    }

    pub(super) async fn set_priorities(&self, regions: &[&str]) -> TestResult {
        self.put_json(&["failover", "priorities"], &json!({ "regions": regions }))
            .await
    }

    pub(super) async fn pause_replication(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "replication", "pause"])
            .await
    }

    pub(super) async fn resume_replication(&self, region: &str) -> TestResult {
        self.post_empty(&["regions", region, "replication", "resume"])
            .await
    }

    pub(super) async fn start_split(
        &self,
        database: &str,
        container: &str,
        partition: u32,
    ) -> TestResult<ManagementOperation> {
        self.post_json_for(
            &[
                "databases",
                database,
                "containers",
                container,
                "partitions",
                &partition.to_string(),
                "split",
            ],
            &json!({ "progressionMode": "manual" }),
        )
        .await
    }

    pub(super) async fn start_merge(
        &self,
        database: &str,
        container: &str,
        partitions: [u32; 2],
    ) -> TestResult<ManagementOperation> {
        self.post_json_for(
            &[
                "databases",
                database,
                "containers",
                container,
                "partitions",
                "merge",
            ],
            &json!({
                "partitionIds": partitions,
                "progressionMode": "manual"
            }),
        )
        .await
    }

    pub(super) async fn advance(&self, operation_id: &str) -> TestResult<ManagementOperation> {
        self.post_empty_for(&["operations", operation_id, "advance"])
            .await
    }

    pub(super) async fn cancel(&self, operation_id: &str) -> TestResult<ManagementOperation> {
        self.post_empty_for(&["operations", operation_id, "cancel"])
            .await
    }

    pub(super) async fn operation(&self, operation_id: &str) -> TestResult<ManagementOperation> {
        self.get_json(&["operations", operation_id]).await
    }

    pub(super) async fn wait_for_succeeded(
        &self,
        operation_id: &str,
    ) -> TestResult<ManagementOperation> {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        loop {
            let operation = self.operation(operation_id).await?;
            match operation.phase.as_str() {
                "Succeeded" => return Ok(operation),
                "Failed" => {
                    return Err(format!(
                        "management operation '{operation_id}' failed: {:?}",
                        operation.error
                    )
                    .into())
                }
                _ if tokio::time::Instant::now() < deadline => {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
                _ => {
                    return Err(format!(
                        "management operation '{operation_id}' did not finish before the deadline"
                    )
                    .into())
                }
            }
        }
    }

    pub(super) async fn reset(&self) -> TestResult {
        let _ = self.complete_failover().await;
        for region in REGIONS {
            let _ = self.cancel_removal(region).await;
            let _ = self.online(region).await;
        }
        let account = self.account().await?;
        for region in REGIONS {
            if !account.regions.iter().any(|entry| entry.name == region) {
                self.add(region).await?;
            }
        }
        self.set_priorities(&REGIONS).await?;
        self.complete_failover().await?;
        for region in REGIONS {
            self.resume_replication(region).await?;
        }
        Ok(())
    }

    async fn post_empty(&self, segments: &[&str]) -> TestResult {
        self.client
            .post(self.url(segments)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn post_json(&self, segments: &[&str], body: &Value) -> TestResult {
        self.client
            .post(self.url(segments)?)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(body)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn put_json(&self, segments: &[&str], body: &Value) -> TestResult {
        self.client
            .put(self.url(segments)?)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(body)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn post_empty_for<T>(&self, segments: &[&str]) -> TestResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .post(self.url(segments)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    async fn post_json_for<T>(&self, segments: &[&str], body: &Value) -> TestResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .post(self.url(segments)?)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(body)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    async fn get_json<T>(&self, segments: &[&str]) -> TestResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .get(self.url(segments)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    }

    fn url(&self, segments: &[&str]) -> TestResult<url::Url> {
        let mut url = self.endpoint.clone();
        url.path_segments_mut()
            .map_err(|_| "management endpoint cannot be a base URL")?
            .extend(segments);
        Ok(url)
    }
}

pub(super) async fn with_manual_operation<T, F>(
    manager: &TopologyManager,
    operation_id: &str,
    operation: F,
) -> TestResult<T>
where
    F: Future<Output = TestResult<T>>,
{
    let outcome = AssertUnwindSafe(operation).catch_unwind().await;
    let cleanup = async {
        let current = manager.operation(operation_id).await;
        let needs_cancel = current.as_ref().map_or(true, |operation| {
            !matches!(operation.phase.as_str(), "Succeeded" | "Failed")
        });
        let probe_error = current.err();
        if needs_cancel {
            if manager.cancel(operation_id).await.is_err() {
                let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
                loop {
                    let current = manager.operation(operation_id).await?;
                    if matches!(current.phase.as_str(), "Succeeded" | "Failed") {
                        break;
                    }
                    if tokio::time::Instant::now() >= deadline {
                        return Err(format!(
                            "manual operation '{operation_id}' did not settle during cleanup"
                        )
                        .into());
                    }
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            }
        }
        if let Some(error) = probe_error {
            return Err(format!(
                "manual operation '{operation_id}' status probe failed before cleanup: {error}"
            )
            .into());
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;
    match outcome {
        Ok(Ok(value)) => {
            cleanup?;
            Ok(value)
        }
        Ok(Err(test_error)) => match cleanup {
            Ok(()) => Err(test_error),
            Err(cleanup_error) => Err(format!(
                "manual topology operation failed: {test_error}; cancellation also failed: {cleanup_error}"
            )
            .into()),
        },
        Err(panic) => {
            if let Err(error) = cleanup {
                eprintln!("manual topology operation cancellation after panic failed: {error}");
            }
            std::panic::resume_unwind(panic)
        }
    }
}

pub(super) async fn with_topology_reset<T, F>(
    manager: &TopologyManager,
    operation: F,
) -> TestResult<T>
where
    F: Future<Output = TestResult<T>>,
{
    manager.reset().await?;
    let outcome = AssertUnwindSafe(operation).catch_unwind().await;
    let reset = manager.reset().await;
    match outcome {
        Ok(Ok(value)) => {
            reset?;
            Ok(value)
        }
        Ok(Err(test_error)) => match reset {
            Ok(()) => Err(test_error),
            Err(reset_error) => Err(format!(
                "topology scenario failed: {test_error}; topology reset also failed: {reset_error}"
            )
            .into()),
        },
        Err(panic) => {
            if let Err(error) = reset {
                eprintln!("topology reset after panic failed: {error}");
            }
            std::panic::resume_unwind(panic)
        }
    }
}

fn management_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("management HTTP client configuration is valid")
    })
}
