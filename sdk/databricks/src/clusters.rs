use crate::client::DatabricksClient;
use crate::prelude::*;
use reqwest::Url;
use serde::Deserialize;
use std::sync::Arc;

const CLUSTER_ENDPOINT: &str = "/api/2.0/clusters";

#[derive(Deserialize, Debug)]
pub struct ClusterList {
    clusters: Vec<Cluster>,
}

#[derive(Deserialize, Debug)]

pub struct ClusterAutoScale {
    pub min_workers: u32,
    pub max_workers: u32,
}

#[derive(Deserialize, Debug)]

pub struct Cluster {
    pub cluster_id: String,
    pub spark_context_id: u64,
    pub spark_version: String,
    pub node_type_id: String,
    pub driver_node_type_id: String,
    pub autotermination_minutes: u32,
    pub enable_elastic_disk: bool,
    pub cluster_source: String,
    pub enable_local_disk_encryption: bool,
    pub state: String,
    pub state_message: String,
    pub start_time: u64,
    pub terminated_time: u64,
    pub last_state_loss_time: u64,
    pub autoscale: ClusterAutoScale,
}

#[derive(Debug)]
pub struct ClusterDriver {
    pub client: Arc<DatabricksClient>,
    endpoint: Url,
}

impl ClusterDriver {
    pub fn new(client: Arc<DatabricksClient>) -> DatabricksResult<ClusterDriver> {
        let endpoint = client.host.join(CLUSTER_ENDPOINT)?;
        Ok(ClusterDriver {
            client: client,
            endpoint: endpoint,
        })
    }

    pub async fn get_list(&self) -> DatabricksResult<ClusterList> {
        let mut base_path = self.endpoint.path().to_string();
        base_path.push_str("/list");

        let list_clusters_url = self.endpoint.join(&base_path)?;
        let list_clusters_url = list_clusters_url;

        let resp = self
            .client
            .as_ref()
            .http_client
            .get(list_clusters_url)
            .send()
            .await?
            .json::<ClusterList>()
            .await?;

        Ok(resp)
    }
}
