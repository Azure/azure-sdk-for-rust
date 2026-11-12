// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::http::headers::{HeaderName, Headers};
use azure_data_cosmos_driver::binary_json::{encode, markers, PREAMBLE};
use azure_data_cosmos_driver::models::{FeedRange, PartitionKey, PartitionKeyDefinition};
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
pub(super) struct Factory(pub(crate) Option<Arc<Gate>>, pub(crate) bool);

impl HttpClientFactory for Factory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(ScriptedTransport(self.0.clone(), self.1)))
    }
}

#[derive(Debug)]
struct ScriptedTransport(Option<Arc<Gate>>, bool);

fn hpk_prefix() -> FeedRange {
    FeedRange::for_partition(
        PartitionKey::from(("tenant", "user")),
        &PartitionKeyDefinition::from(("/tenant", "/user", "/device")),
    )
}

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

fn referenced_feed(body: &serde_json::Value) -> Vec<u8> {
    fn object(fields: &[(&str, Vec<u8>)]) -> Vec<u8> {
        let mut payload = Vec::new();
        for (name, value) in fields {
            payload.extend(&encode(&serde_json::json!(name))[1..]);
            payload.extend(value);
        }
        frame(markers::OBJ_LC4, fields.len(), payload)
    }
    fn frame(marker: u8, count: usize, payload: Vec<u8>) -> Vec<u8> {
        let mut bytes = vec![marker];
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(count as u32).to_le_bytes());
        bytes.extend(payload);
        bytes
    }
    let target = 10 + encode(&serde_json::json!("shared")).len() - 1;
    let mut reference = vec![markers::STR_R4];
    reference.extend_from_slice(&(target as u32).to_le_bytes());
    let documents = body["Documents"].as_array().unwrap();
    let rows: Vec<u8> = documents
        .iter()
        .flat_map(|row| {
            let payload = object(&[
                ("id", encode(&row["payload"]["id"])[1..].to_vec()),
                ("rank", encode(&row["payload"]["rank"])[1..].to_vec()),
                ("shared", reference.clone()),
            ]);
            object(&[
                ("_rid", encode(&row["_rid"])[1..].to_vec()),
                ("orderByItems", encode(&row["orderByItems"])[1..].to_vec()),
                ("payload", payload),
            ])
        })
        .collect();
    let mut bytes = vec![PREAMBLE];
    bytes.extend(object(&[
        (
            "shared",
            encode(&serde_json::json!("reference outside every document"))[1..].to_vec(),
        ),
        ("Documents", frame(markers::ARR_LC4, documents.len(), rows)),
    ]));
    bytes
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
            "/dbs/testdb/colls/testcoll" if self.1 => serde_json::json!({
                "id":"testcoll", "_rid":"AQIDBAUGBwg=",
                "partitionKey":{"paths":["/tenant","/user","/device"],"kind":"MultiHash","version":2}
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
                let boundary = if self.1 {
                    format!("{}80", hpk_prefix().min_inclusive().to_hex())
                } else {
                    "80".to_owned()
                };
                serde_json::json!({"PartitionKeyRanges":[
                    {"id":"0","minInclusive":"","maxExclusive":boundary},
                    {"id":"1","minInclusive":boundary,"maxExclusive":"FF"}
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
                    if self.1 {
                        assert!(header(request, "x-ms-documentdb-partitionkey").is_none());
                        let min =
                            header(request, "x-ms-start-epk").expect("prefix EPK lower bound");
                        let max = header(request, "x-ms-end-epk").expect("prefix EPK upper bound");
                        assert!(!min.is_empty() && max != "FF" && min < max);
                    }
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
                            let range = if self.1 {
                                header(request, "x-ms-documentdb-partitionkeyrangeid").unwrap()
                            } else {
                                ""
                            };
                            let doc = serde_json::json!({"id":format!("{range}d{rank}"),"rank":rank,"Documents":["application-field"]});
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
        let binary_response = request.url.path().ends_with("/docs")
            && header(request, "x-ms-cosmos-is-query-plan-request").is_none()
            && header(request, "a-im").is_none()
            && header(request, "x-ms-cosmos-supported-serialization-formats")
                .is_some_and(|formats| formats.contains("CosmosBinary"));
        Ok(HttpResponse {
            status,
            headers,
            body: if status == 304 {
                Vec::new()
            } else if binary_response {
                if body["Documents"].as_array().is_some_and(|documents| {
                    documents
                        .iter()
                        .any(|row| row.get("orderByItems").is_some())
                }) {
                    referenced_feed(&body)
                } else {
                    encode(&body)
                }
            } else {
                serde_json::to_vec(&body).unwrap()
            },
        })
    }
}
