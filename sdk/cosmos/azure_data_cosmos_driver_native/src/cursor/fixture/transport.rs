// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::http::headers::{HeaderName, Headers};
use azure_data_cosmos_driver::test::{
    ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
    TransportClient, TransportError,
};
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

#[derive(Debug, Default)]
pub(crate) struct Gate {
    pub(crate) entered: tokio::sync::Notify,
    pub(crate) release: tokio::sync::Notify,
    pub(crate) failure: AtomicU8,
}

#[derive(Debug)]
pub(super) struct Factory(pub(crate) Option<Arc<Gate>>);

impl HttpClientFactory for Factory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(ScriptedTransport(self.0.clone())))
    }
}

#[derive(Debug)]
struct ScriptedTransport(Option<Arc<Gate>>);

fn header<'a>(request: &'a HttpRequest, name: &'static str) -> Option<&'a str> {
    request
        .headers
        .get_optional_str(&HeaderName::from_static(name))
}

#[async_trait]
impl TransportClient for ScriptedTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        tokio::task::yield_now().await;
        if request.url.path().ends_with("/docs")
            && header(request, "x-ms-cosmos-is-query-plan-request").is_none()
        {
            if let Some(gate) = &self.0 {
                gate.entered.notify_one();
                gate.release.notified().await;
                match gate.failure.load(Ordering::Acquire) {
                    1 => panic!("scripted transport panic"),
                    2 => {
                        return Ok(HttpResponse {
                            status: 400,
                            headers: Headers::new(),
                            body:
                                br#"{"code":"BadRequest","message":"scripted execution failure"}"#
                                    .to_vec(),
                        })
                    }
                    _ => {}
                }
            }
        }
        let mut headers = Headers::new();
        let mut status = 200;
        let body = match request.url.path() {
            "/" => serde_json::json!({
                "id":"cursor-fixture", "_rid":"cursor-fixture", "_self":"",
                "writableLocations":[{"name":"East US","databaseAccountEndpoint":"https://eastus.emulator.local/"}],
                "readableLocations":[{"name":"East US","databaseAccountEndpoint":"https://eastus.emulator.local/"}],
                "enableMultipleWriteLocations":false,
                "userConsistencyPolicy":{"defaultConsistencyLevel":"Session"}
            }),
            "/dbs/testdb/colls/testcoll" => serde_json::json!({
                "id":"testcoll", "_rid":"AQIDBAUGBwg=",
                "partitionKey":{"paths":["/pk"],"kind":"Hash","version":2}
            }),
            path if path.ends_with("/pkranges") => {
                if header(request, "if-none-match").is_some() {
                    status = 304;
                }
                headers.insert("etag", "\"ranges\"");
                serde_json::json!({"PartitionKeyRanges":[
                    {"id":"0","minInclusive":"","maxExclusive":"80"},
                    {"id":"1","minInclusive":"80","maxExclusive":"FF"}
                ],"_count":2})
            }
            path if path.ends_with("/docs") => {
                if header(request, "x-ms-cosmos-is-query-plan-request") == Some("True")
                    || header(request, "x-ms-cosmos-is-query-plan-request") == Some("true")
                {
                    serde_json::json!({
                        "partitionedQueryExecutionInfoVersion":1,
                        "queryInfo":{"hasNonStreamingOrderBy":true,"orderBy":["Ascending"],"orderByExpressions":["c.rank"],"top":6,"rewrittenQuery":"SELECT TOP 6 * FROM c ORDER BY c.rank"},
                        "queryRanges":[{"min":"","max":"FF","isMinInclusive":true,"isMaxInclusive":false}]
                    })
                } else if let Some(mode) = header(request, "a-im") {
                    let continuation = header(request, "if-none-match");
                    if mode == "Full-Fidelity Feed"
                        && (continuation.is_none()
                            || header(request, "if-modified-since").is_some())
                    {
                        status = 400;
                        serde_json::json!({"code":"BadRequest","message":"unsupported AllVersionsAndDeletes start"})
                    } else if continuation == Some("*") || continuation == Some("\"done\"") {
                        status = 304;
                        headers.insert(
                            "etag",
                            if continuation == Some("*") {
                                "\"idle\""
                            } else {
                                "\"done\""
                            },
                        );
                        serde_json::Value::Null
                    } else {
                        headers.insert("etag", "\"done\"");
                        let docs: Vec<_> = [0, 1].into_iter().map(|rank| {
                            let doc = serde_json::json!({"id":format!("d{rank}"),"rank":rank,"Documents":["application-field"]});
                            if mode == "Full-Fidelity Feed" {
                                serde_json::json!({"current":doc,"previous":{"id":format!("d{rank}"),"rank":-1},"metadata":{"operationType":"replace","lsn":rank+1}})
                            } else { doc }
                        }).collect();
                        serde_json::json!({"Documents":docs,"_count":2})
                    }
                } else {
                    let ranks =
                        if header(request, "x-ms-documentdb-partitionkeyrangeid") == Some("0") {
                            [5, 1, 3]
                        } else {
                            [4, 0, 2]
                        };
                    let start: usize = header(request, "x-ms-continuation")
                        .unwrap_or("0")
                        .parse()
                        .unwrap();
                    let count: usize = header(request, "x-ms-max-item-count")
                        .unwrap_or("2")
                        .parse()
                        .unwrap();
                    let end = (start + count).min(ranks.len());
                    if end < ranks.len() {
                        headers.insert("x-ms-continuation", end.to_string());
                    }
                    let docs: Vec<_> = ranks[start..end]
                        .iter()
                        .map(|rank| {
                            serde_json::json!({
                                "_rid":format!("AQIDBAUGBwg{rank}="),"orderByItems":[{"item":rank}],
                                "payload":{"id":format!("d{rank}"),"rank":rank}
                            })
                        })
                        .collect();
                    serde_json::json!({"Documents":docs,"_count":end-start})
                }
            }
            path => panic!("unexpected scripted request {path}"),
        };
        headers.insert("x-ms-request-charge", "1.5");
        Ok(HttpResponse {
            status,
            headers,
            body: if status == 304 {
                Vec::new()
            } else {
                serde_json::to_vec(&body).unwrap()
            },
        })
    }
}
