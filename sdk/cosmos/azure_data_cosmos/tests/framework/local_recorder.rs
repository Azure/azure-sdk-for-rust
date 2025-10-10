use std::sync::Arc;

use azure_core::http::{
    policies::{Policy, PolicyResult},
    BufResponse, Context, RawResponse, Request,
};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub request: Request,
    pub response: Option<RawResponse>,
}

/// A policy that can be used to capture a simple local recording of requests for validation purposes
pub struct LocalRecorder {
    transactions: tokio::sync::RwLock<Vec<Transaction>>,
}

impl LocalRecorder {
    pub fn new() -> Self {
        Self {
            transactions: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    /// Returns a copy of all recorded transactions
    pub async fn to_transactions(&self) -> Vec<Transaction> {
        self.transactions.read().await.clone()
    }
}

impl std::fmt::Debug for LocalRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalRecorder").finish()
    }
}

#[async_trait::async_trait]
impl Policy for LocalRecorder {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let response = next[0].send(ctx, request, &next[1..]).await?;
        let (status, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let raw_response = RawResponse::from_bytes(status, headers.clone(), body.clone());
        self.transactions.write().await.push(Transaction {
            request: request.clone(),
            response: Some(raw_response.clone()),
        });
        Ok(BufResponse::from_bytes(status, headers, body))
    }
}
