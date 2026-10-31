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

#[derive(Clone, Copy)]
enum QueryScenario {
    Aggregate(&'static str),
    Aliases,
    GroupBy,
    DistinctCount,
}

fn query_scenario(sql: &str) -> Option<QueryScenario> {
    if sql.contains("FROM (SELECT DISTINCT VALUE c.category FROM c") {
        Some(QueryScenario::DistinctCount)
    } else if sql.contains("GROUP BY c.category") {
        Some(QueryScenario::GroupBy)
    } else if sql.starts_with("SELECT COUNT(1) AS count, SUM(c.amount) AS total") {
        Some(QueryScenario::Aliases)
    } else {
        [
            ("COUNT(1)", "Count"),
            ("SUM(c.amount)", "Sum"),
            ("MIN(c.amount)", "Min"),
            ("MAX(c.amount)", "Max"),
            ("AVG(c.amount)", "Average"),
        ]
        .into_iter()
        .find_map(|(expression, operator)| {
            sql.starts_with(&format!("SELECT VALUE {expression} FROM c"))
                .then_some(QueryScenario::Aggregate(operator))
        })
        .or_else(|| {
            sql.starts_with("SELECT DISTINCT VALUE c.category FROM c")
                .then_some(QueryScenario::DistinctCount)
        })
    }
}

fn query_info(scenario: QueryScenario, sql: &str) -> serde_json::Value {
    match scenario {
        QueryScenario::Aggregate(operator) => serde_json::json!({
            "aggregates":[operator], "hasSelectValue":true, "rewrittenQuery":sql
        }),
        QueryScenario::Aliases => serde_json::json!({
            "groupByAliases":["count","total","min","max","avg"],
            "groupByAliasToAggregateType":{
                "count":"Count","total":"Sum","min":"Min","max":"Max","avg":"Average"
            },
            "rewrittenQuery":sql
        }),
        QueryScenario::GroupBy => serde_json::json!({
            "groupByExpressions":["c.category"],
            "groupByAliases":["category","count","total","min","max","avg"],
            "groupByAliasToAggregateType":{
                "category":null,"count":"Count","total":"Sum","min":"Min","max":"Max","avg":"Average"
            },
            "rewrittenQuery":sql
        }),
        QueryScenario::DistinctCount => serde_json::json!({
            "dCountInfo":{"dCountAlias":null}, "distinctType":"Unordered",
            "hasSelectValue":true,
            "rewrittenQuery": if sql.contains("WHERE false") {
                "SELECT DISTINCT VALUE c.category FROM c WHERE false"
            } else {
                "SELECT DISTINCT VALUE c.category FROM c"
            }
        }),
    }
}

fn query_documents(
    scenario: QueryScenario,
    first_range: bool,
    empty: bool,
) -> Vec<serde_json::Value> {
    if empty {
        return match scenario {
            QueryScenario::Aggregate("Count") => vec![serde_json::json!([{"item":0}])],
            _ => Vec::new(),
        };
    }
    let partials = if first_range {
        [(2, 30, 10, 20), (1, 30, 30, 30)]
    } else {
        [(1, 5, 5, 5), (2, 115, 15, 100)]
    };
    match scenario {
        QueryScenario::Aggregate(operator) => partials
            .into_iter()
            .map(|(count, sum, min, max)| {
                let value = match operator {
                    "Count" => serde_json::json!(count),
                    "Sum" => serde_json::json!(sum),
                    "Min" => serde_json::json!(min),
                    "Max" => serde_json::json!(max),
                    "Average" => serde_json::json!({"sum":sum,"count":count}),
                    _ => unreachable!(),
                };
                serde_json::json!([{"item":value}])
            })
            .collect(),
        QueryScenario::Aliases => partials
            .into_iter()
            .map(|(count, sum, min, max)| {
                serde_json::json!({"payload":{
                    "count":{"item":count}, "total":{"item":sum},
                    "min":{"item":min}, "max":{"item":max},
                    "avg":{"item":{"sum":sum,"count":count}}
                }})
            })
            .collect(),
        QueryScenario::GroupBy => {
            let groups = if first_range {
                vec![("a", 2, 30, 10, 20), ("b", 1, 5, 5, 5)]
            } else {
                vec![
                    ("a", 1, 30, 30, 30),
                    ("b", 1, 15, 15, 15),
                    ("c", 1, 100, 100, 100),
                ]
            };
            groups
                .into_iter()
                .map(|(category, count, sum, min, max)| {
                    serde_json::json!({
                        "groupByItems":[{"item":category}],
                        "payload":{
                            "category":category,"count":{"item":count},"total":{"item":sum},
                            "min":{"item":min},"max":{"item":max},
                            "avg":{"item":{"sum":sum,"count":count}}
                        }
                    })
                })
                .collect()
        }
        QueryScenario::DistinctCount => {
            let categories = if first_range {
                ["a", "b", "a"]
            } else {
                ["a", "c", "b"]
            };
            categories
                .into_iter()
                .map(|category| serde_json::json!(category))
                .collect()
        }
    }
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
                let query: serde_json::Value = request
                    .body
                    .as_ref()
                    .and_then(|body| serde_json::from_slice(body).ok())
                    .unwrap_or_default();
                let sql = query["query"].as_str().unwrap_or_default();
                let scenario = query_scenario(sql);
                if header(request, "x-ms-cosmos-is-query-plan-request") == Some("True")
                    || header(request, "x-ms-cosmos-is-query-plan-request") == Some("true")
                {
                    if let Some(scenario) = scenario {
                        serde_json::json!({
                            "partitionedQueryExecutionInfoVersion":1,
                            "queryInfo":query_info(scenario, sql),
                            "queryRanges":[{"min":"","max":"FF","isMinInclusive":true,"isMaxInclusive":false}]
                        })
                    } else {
                        serde_json::json!({
                            "partitionedQueryExecutionInfoVersion":1,
                            "queryInfo":{"hasNonStreamingOrderBy":true,"orderBy":["Ascending"],"orderByExpressions":["c.rank"],"top":6,"rewrittenQuery":"SELECT TOP 6 * FROM c ORDER BY c.rank"},
                            "queryRanges":[{"min":"","max":"FF","isMinInclusive":true,"isMaxInclusive":false}]
                        })
                    }
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
                } else if let Some(scenario) = scenario {
                    let docs = query_documents(
                        scenario,
                        header(request, "x-ms-documentdb-partitionkeyrangeid") == Some("0"),
                        sql.contains("WHERE false"),
                    );
                    let start: usize = header(request, "x-ms-continuation")
                        .unwrap_or("0")
                        .parse()
                        .unwrap();
                    let count: usize = header(request, "x-ms-max-item-count")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(docs.len());
                    let end = start.saturating_add(count).min(docs.len());
                    if end < docs.len() {
                        headers.insert("x-ms-continuation", end.to_string());
                    }
                    serde_json::json!({"Documents":docs[start..end],"_count":end-start})
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
