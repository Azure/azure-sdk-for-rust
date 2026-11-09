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
    read_many::{invalid, ItemScope, ReadManyRequest},
};

use super::{
    PageResult, PartitionRoutingRefresh, Pipeline, PipelineContext, PipelineNode,
    PipelineNodeState, Request, RequestTarget, SequentialDrain, TopologyProvider,
};

// Stay below the service's SQL-text and request-body limits, including parameters.
const MAX_BATCH_BYTES: usize = 256 * 1024;
const MAX_BATCH_SELECTIONS: usize = 64;

#[derive(Clone, azure_core::fmt::SafeDebug, PartialEq, Eq)]
struct Identity {
    partition_key: PartitionKey,
    item_id: Option<String>,
}

pub(crate) async fn build(
    operation: Arc<CosmosOperation>,
    request: &ReadManyRequest,
    topology: &mut dyn TopologyProvider,
) -> crate::Result<Pipeline> {
    let container = operation
        .container()
        .ok_or_else(|| invalid("read-many requires a container"))?;
    let definition = container.partition_key_definition();
    if let ItemScope::FeedRanges(scopes) = &request.selection {
        return build_feed_ranges(operation.clone(), request, scopes, topology).await;
    }
    let identities = normalize(request.selection.clone(), definition)?;
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
        let epk = FeedRange::for_partition(identity.partition_key.clone(), definition);
        let matching = ranges
            .iter()
            .enumerate()
            .filter(|(_, range)| {
                if identity.partition_key.len() == definition.paths().len() {
                    range.range.min_inclusive() <= epk.min_inclusive()
                        && epk.min_inclusive() < range.range.max_exclusive()
                } else {
                    epk.overlaps(&range.range)
                }
            })
            .collect::<Vec<_>>();
        if matching.is_empty() {
            return Err(invalid(
                "read-many partition key is not covered by container topology",
            ));
        }
        if identity.partition_key.len() < definition.paths().len() {
            let covered = matching
                .iter()
                .map(|(_, range)| {
                    super::intersect_feed_ranges(&epk, &range.range)
                        .ok_or_else(|| invalid("invalid read-many prefix topology coverage"))
                })
                .collect::<crate::Result<Vec<_>>>()?;
            super::validate_exact_coverage(&epk, covered.iter())?;
        }
        for (index, _) in matching {
            groups.entry(index).or_default().push(identity.clone());
        }
    }
    let width = groups.len();
    let mut children: Vec<Box<dyn PipelineNode>> = Vec::new();
    for (index, identities) in groups {
        let range = &ranges[index];
        if identities.len() == 1
            && identities[0].partition_key.len() == definition.paths().len()
            && filter.is_none()
            && !container.is_by_rid()
        {
            if let Identity {
                partition_key: pk,
                item_id: Some(id),
            } = &identities[0]
            {
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
    selection: ItemScope,
    definition: &PartitionKeyDefinition,
) -> crate::Result<Vec<Identity>> {
    let mut identities = match selection {
        ItemScope::Items(items) => items
            .into_iter()
            .map(|(partition_key, id)| Identity {
                partition_key,
                item_id: Some(id),
            })
            .collect::<Vec<_>>(),
        ItemScope::Partitions(keys) => keys
            .into_iter()
            .map(|partition_key| Identity {
                partition_key,
                item_id: None,
            })
            .collect(),
        ItemScope::FeedRanges(_) => return Err(invalid("feed ranges require range planning")),
    };
    for Identity {
        partition_key: pk,
        item_id: id,
    } in &mut identities
    {
        if let Some(id) = id {
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
                *pk = values.try_into()?;
            }
        }
        if pk.is_empty()
            || pk.len() > definition.paths().len()
            || (pk.len() < definition.paths().len()
                && definition.kind() != PartitionKeyKind::MultiHash)
        {
            return Err(invalid(
                "read-many requires a complete partition key or a nonempty hierarchical prefix",
            ));
        }
        if pk.values().iter().any(|value| value.is_infinity()) {
            return Err(invalid("invalid partition key sentinel"));
        }
    }
    // Keep borrowed lookup keys only until all overlap decisions are complete.
    let keep = {
        let mut seen = HashSet::new();
        let first = identities
            .iter()
            .map(|identity| {
                seen.insert((identity.partition_key.values(), identity.item_id.as_deref()))
            })
            .collect::<Vec<_>>();
        identities
            .iter()
            .zip(first)
            .map(|(identity, first)| {
                first
                    && !(1..identity.partition_key.len()).any(|length| {
                        seen.contains(&(
                            &identity.partition_key.values()[..length],
                            identity.item_id.as_deref(),
                        ))
                    })
            })
            .collect::<Vec<_>>()
    };
    Ok(identities
        .into_iter()
        .zip(keep)
        .filter_map(|(identity, keep)| keep.then_some(identity))
        .collect())
}

fn normalize_feed_ranges(
    scopes: &[FeedRange],
    definition: &PartitionKeyDefinition,
) -> crate::Result<Vec<FeedRange>> {
    let width = crate::models::effective_partition_key::normalized_epk_len(definition);
    let mut scopes = scopes
        .iter()
        .map(|scope| {
            let scope = if let Some(key) = scope.partition_key() {
                normalize(ItemScope::Partitions(vec![key.clone()]), definition)?;
                FeedRange::for_partition(key.clone(), definition)
            } else {
                scope.clone()
            };
            let min = scope.min_inclusive().clone();
            let max = if &min == scope.max_exclusive() {
                // Transport EPK filters use half-open intervals, including point selections.
                width.map_or_else(|| min.successor(), |width| min.normalized_successor(width))
            } else {
                scope.max_exclusive().clone()
            };
            if min >= max || max > crate::models::EffectivePartitionKey::MAX {
                return Err(invalid(
                    "read-many feed range is outside the partition key space",
                ));
            }
            FeedRange::new(min, max)
        })
        .collect::<crate::Result<Vec<_>>>()?;
    scopes.sort_by(|left, right| {
        left.min_inclusive()
            .cmp(right.min_inclusive())
            .then_with(|| right.max_exclusive().cmp(left.max_exclusive()))
    });
    let mut merged: Vec<FeedRange> = Vec::new();
    for scope in scopes {
        if let Some(last) = merged.last_mut() {
            if scope.min_inclusive() <= last.max_exclusive() {
                *last = FeedRange::new(
                    last.min_inclusive().clone(),
                    last.max_exclusive().max(scope.max_exclusive()).clone(),
                )?;
                continue;
            }
        }
        merged.push(scope);
    }
    Ok(merged)
}

async fn build_feed_ranges(
    operation: Arc<CosmosOperation>,
    request: &ReadManyRequest,
    scopes: &[FeedRange],
    topology: &mut dyn TopologyProvider,
) -> crate::Result<Pipeline> {
    let filter = request
        .filter
        .as_ref()
        .map(|f| f.query_predicate())
        .transpose()?;
    let parameters = request.filter.as_ref().map(|f| &f.parameters);
    let query = match filter {
        Some(filter) => format!("SELECT * FROM c WHERE ({filter})"),
        None => "SELECT * FROM c".to_owned(),
    };
    let parameters = parameters
        .into_iter()
        .flat_map(|p| p.iter())
        .map(|(name, value)| json!({"name": name, "value": value}))
        .collect::<Vec<_>>();
    let body = serde_json::to_vec(&json!({"query":query,"parameters":parameters}))?;
    if body.len() > MAX_BATCH_BYTES {
        return Err(invalid("read-many filter exceeds the query batch size"));
    }
    let definition = operation
        .container()
        .ok_or_else(|| invalid("read-many requires a container"))?
        .partition_key_definition();
    let scopes = normalize_feed_ranges(scopes, definition)?;
    let query_operation = Arc::new(operation.as_ref().clone().with_body(body));
    let mut children: Vec<Box<dyn PipelineNode>> = Vec::new();
    let mut physical_ids = HashSet::new();
    for scope in scopes {
        let ranges = topology
            .resolve_ranges(&scope, PartitionRoutingRefresh::UseCached)
            .await?;
        let mut covered = Vec::new();
        for range in ranges {
            let clipped = super::intersect_feed_ranges(&scope, &range.range);
            let Some(clipped) = clipped else { continue };
            covered.push(clipped.clone());
            physical_ids.insert(range.partition_key_range_id.clone());
            let target = RequestTarget::effective_partition_key_range_with_parents(
                clipped,
                range.partition_key_range_id,
                range.parents,
                range.range,
            );
            children.push(Box::new(Request::new(
                Arc::clone(&query_operation),
                target,
                None,
            )));
        }
        covered.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
        super::validate_exact_coverage(&scope, covered.iter())?;
    }
    Ok(Pipeline::new(Box::new(ReadManyDrain {
        inner: SequentialDrain::new(children),
        width: physical_ids.len(),
        emit_binary: operation.emits_binary_payload(),
        failure: None,
    })))
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
    for Identity {
        partition_key: key,
        item_id: id,
    } in identities
    {
        let mut terms = Vec::new();
        if let Some(id) = id {
            terms.push(format!("c.id = {}", bind(Value::String(id.clone()))));
        }
        for (selector, value) in selectors.iter().zip(key.values()) {
            terms.push(match value.query_value()? {
                Some(value) => format!("{selector} = {}", bind(value)),
                // Empty objects are the only valid object-valued partition keys.
                None => format!("(NOT IS_DEFINED({selector}) OR IS_OBJECT({selector}))"),
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
