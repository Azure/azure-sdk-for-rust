// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;
use azure_core::http::StatusCode;
use serde_json::{json, Value};

use crate::{
    models::{
        CosmosOperation, CosmosResponse, CosmosStatus, FeedRange, ItemReference, PartitionKey,
        PartitionKeyDefinition, PartitionKeyKind, ResponseBody,
    },
    read_many::{invalid, ReadManyRequest, ReadManySelection},
};

use super::{
    PageResult, PartitionRoutingRefresh, Pipeline, PipelineContext, PipelineNode,
    PipelineNodeState, Request, RequestTarget, SequentialDrain, TopologyProvider,
};

// Stay below the service's SQL-text and request-body limits, including parameters.
const MAX_BATCH_BYTES: usize = 256 * 1024;
const MAX_BATCH_SELECTIONS: usize = 64;

type Identity = (PartitionKey, Option<String>);

pub(crate) async fn build(
    operation: Arc<CosmosOperation>,
    request: &ReadManyRequest,
    topology: &mut dyn TopologyProvider,
) -> crate::Result<Pipeline> {
    let container = operation
        .container()
        .ok_or_else(|| invalid("read-many requires a container"))?;
    let definition = container.partition_key_definition();
    let identities = normalize(&request.selection, definition)?;
    let filter = request
        .filter
        .as_ref()
        .map(|f| f.query_predicate())
        .transpose()?;
    let selectors = selectors(definition)?;
    if identities.is_empty() {
        return Ok(Pipeline::new(Box::new(ReadManyDrain {
            inner: SequentialDrain::new(Vec::new()),
            width: 0,
            emit_binary: operation.emits_binary_payload(),
            failure: None,
        })));
    }
    let ranges = topology
        .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::UseCached)
        .await?;
    let mut groups: BTreeMap<usize, Vec<Identity>> = BTreeMap::new();
    for identity in identities {
        let epk = FeedRange::for_partition(identity.0.clone(), definition);
        let index = ranges
            .iter()
            .position(|range| epk.is_subset_of(&range.range))
            .ok_or_else(|| {
                invalid("read-many partition key is not covered by container topology")
            })?;
        groups.entry(index).or_default().push(identity);
    }
    let width = groups.len();
    let mut children: Vec<Box<dyn PipelineNode>> = Vec::new();
    for (index, identities) in groups {
        let range = &ranges[index];
        if identities.len() == 1 && filter.is_none() {
            if let (pk, Some(id)) = &identities[0] {
                let item = ItemReference::from_name(container, pk.clone(), id.clone());
                let mut read = CosmosOperation::read_item(item)
                    .with_request_headers(operation.request_headers().clone());
                read.read_many = operation.read_many.clone();
                children.push(Box::new(Singleton {
                    request: Request::new(
                        Arc::new(read),
                        RequestTarget::logical_partition_key_with_parents(
                            pk.clone(),
                            range.partition_key_range_id.clone(),
                            range.parents.clone(),
                        ),
                        None,
                    ),
                }));
                continue;
            }
        }
        for chunk in identities.chunks(MAX_BATCH_SELECTIONS) {
            let mut offset = 0;
            while offset < chunk.len() {
                let mut end = chunk.len();
                let body = loop {
                    let body =
                        query_body(&chunk[offset..end], &selectors, request, filter.as_deref())?;
                    if body.len() <= MAX_BATCH_BYTES {
                        break body;
                    }
                    if end == offset + 1 {
                        return Err(invalid(
                            "one read-many selection and filter exceed the query batch size",
                        ));
                    }
                    end = offset + (end - offset) / 2;
                };
                let query = Arc::new(operation.as_ref().clone().with_body(body));
                children.push(Box::new(Request::new(
                    query,
                    RequestTarget::effective_partition_key_range_with_parents(
                        range.range.clone(),
                        range.partition_key_range_id.clone(),
                        range.parents.clone(),
                        range.range.clone(),
                    ),
                    None,
                )));
                offset = end;
            }
        }
    }
    Ok(Pipeline::new(Box::new(ReadManyDrain {
        inner: SequentialDrain::new(children),
        width,
        emit_binary: operation.emits_binary_payload(),
        failure: None,
    })))
}

fn normalize(
    selection: &ReadManySelection,
    definition: &PartitionKeyDefinition,
) -> crate::Result<Vec<Identity>> {
    let identities = match selection {
        ReadManySelection::Items(items) => items
            .iter()
            .map(|(pk, id)| (pk.clone(), Some(id.clone())))
            .collect::<Vec<_>>(),
        ReadManySelection::Partitions(keys) => keys.iter().cloned().map(|pk| (pk, None)).collect(),
    };
    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for (mut pk, id) in identities {
        if let Some(id) = &id {
            if id.is_empty() {
                return Err(invalid("read-many item ids must not be empty"));
            }
            if definition.kind() == PartitionKeyKind::MultiHash
                && pk.len() + 1 == definition.paths().len()
                && definition
                    .paths()
                    .last()
                    .is_some_and(|p| p.as_ref() == "/id")
            {
                let mut values = pk.values().to_vec();
                values.push(id.clone().into());
                pk = values.try_into()?;
            }
        }
        if pk.is_empty() || pk.len() != definition.paths().len() {
            return Err(invalid(
                "read-many requires complete logical partition keys",
            ));
        }
        for value in pk.values() {
            value.query_value()?;
        }
        if seen.insert((pk.clone(), id.clone())) {
            unique.push((pk, id));
        }
    }
    Ok(unique)
}

fn selectors(definition: &PartitionKeyDefinition) -> crate::Result<Vec<String>> {
    definition
        .paths()
        .iter()
        .map(|path| {
            let segments = crate::models::partition_key::parse_path(path)?;
            let mut selector = String::from("c");
            for segment in segments {
                selector.push('[');
                selector.push_str(&serde_json::to_string(&segment)?);
                selector.push(']');
            }
            Ok(selector)
        })
        .collect()
}

fn query_body(
    identities: &[Identity],
    selectors: &[String],
    request: &ReadManyRequest,
    filter: Option<&str>,
) -> crate::Result<Vec<u8>> {
    let mut parameters = request
        .filter
        .as_ref()
        .map(|filter| filter.parameters.clone())
        .unwrap_or_default();
    let mut next = 0;
    let mut bind = |value: Value| {
        let name = loop {
            let name = format!("@__read_many_{next}");
            next += 1;
            if !parameters.contains_key(&name) {
                break name;
            }
        };
        parameters.insert(name.clone(), value);
        name
    };
    let mut predicates = Vec::new();
    for (key, id) in identities {
        let mut terms = Vec::new();
        if let Some(id) = id {
            terms.push(format!("c.id = {}", bind(Value::String(id.clone()))));
        }
        for (selector, value) in selectors.iter().zip(key.values()) {
            terms.push(match value.query_value()? {
                Some(value) => format!("{selector} = {}", bind(value)),
                None => format!("NOT IS_DEFINED({selector})"),
            });
        }
        predicates.push(format!("({})", terms.join(" AND ")));
    }
    let mut query = format!("SELECT * FROM c WHERE ({})", predicates.join(" OR "));
    if let Some(filter) = filter {
        query.push_str(&format!(" AND ({filter})"));
    }
    let parameters = parameters
        .into_iter()
        .map(|(name, value)| json!({"name":name,"value":value}))
        .collect::<Vec<_>>();
    Ok(serde_json::to_vec(
        &json!({"query":query,"parameters":parameters}),
    )?)
}

struct ReadManyDrain {
    inner: SequentialDrain,
    width: usize,
    emit_binary: bool,
    failure: Option<crate::CosmosError>,
}

#[async_trait]
impl PipelineNode for ReadManyDrain {
    async fn next_page(&mut self, context: &mut PipelineContext<'_>) -> crate::Result<PageResult> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let result = self.next_page_inner(context).await;
        if let Err(error) = &result {
            self.failure = Some(error.clone());
        }
        result
    }
    fn snapshot_state(&self) -> crate::Result<PipelineNodeState> {
        Err(crate::CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED)
            .with_message(
                "read-many does not support durable checkpoints; continue the live cursor",
            )
            .build())
    }
    fn fan_out_width(&self) -> usize {
        self.width
    }
    fn topology_can_change(&self) -> bool {
        false
    }
    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![Box::new(self.inner)]
    }
}

impl ReadManyDrain {
    async fn next_page_inner(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::Result<PageResult> {
        match self.inner.next_page(context).await? {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                let mut headers = response.headers().clone();
                headers.continuation = None;
                let diagnostics = Arc::new(
                    response
                        .diagnostics_ref()
                        .clone_with_operation_name(Some(Arc::from("read_many")))
                        .with_operation_status(CosmosStatus::new(StatusCode::Ok)),
                );
                let items = match response.into_body() {
                    ResponseBody::Bytes(bytes) => {
                        super::skip_take_page::split_feed_envelope(&bytes).and_then(|items| {
                            super::skip_take_page::encode_items(items, self.emit_binary)
                        })
                    }
                    ResponseBody::Items(items) => Ok(items),
                    ResponseBody::NoPayload => Ok(Vec::new()),
                }
                .map_err(|error| {
                    crate::CosmosErrorBuilder::from_error(error)
                        .with_diagnostics(diagnostics.clone())
                        .build()
                })?;
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        ResponseBody::Items(items),
                        headers,
                        CosmosStatus::new(StatusCode::Ok),
                        diagnostics,
                    ),
                    is_terminal,
                })
            }
            result => Ok(result),
        }
    }
}

struct Singleton {
    request: Request,
}

#[async_trait]
impl PipelineNode for Singleton {
    async fn next_page(&mut self, context: &mut PipelineContext<'_>) -> crate::Result<PageResult> {
        match self.request.next_page(context).await {
            Ok(PageResult::Page {
                response,
                is_terminal,
            }) => {
                let headers = response.headers().clone();
                let diagnostics = response.diagnostics();
                let item = response.into_body().single()?;
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        ResponseBody::Items(vec![item]),
                        headers,
                        CosmosStatus::new(StatusCode::Ok),
                        diagnostics,
                    ),
                    is_terminal,
                })
            }

            Err(error)
                if error.status().status_code() == StatusCode::NotFound
                    && error.status().sub_status().is_none_or(|s| s.value() == 0) =>
            {
                let diagnostics = error.diagnostics().ok_or_else(|| error.clone())?;
                let headers = error
                    .response()
                    .ok_or_else(|| error.clone())?
                    .headers()
                    .clone();
                Ok(PageResult::Page {
                    response: CosmosResponse::new(
                        ResponseBody::Items(Vec::new()),
                        headers,
                        CosmosStatus::new(StatusCode::Ok),
                        diagnostics,
                    ),
                    is_terminal: true,
                })
            }
            result => result,
        }
    }
    fn snapshot_state(&self) -> crate::Result<PipelineNodeState> {
        self.request.snapshot_state()
    }
    fn fan_out_width(&self) -> usize {
        1
    }
    fn topology_can_change(&self) -> bool {
        false
    }
    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![Box::new(self.request)]
    }
}

#[cfg(test)]
mod tests;
