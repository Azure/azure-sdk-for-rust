// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{collections::VecDeque, fmt::Debug, str::FromStr};

use crate::{query::aggregators::Aggregator, ErrorKind};

use super::{producer::ItemProducer, QueryResult};

#[derive(Debug)]
pub struct PipelineNodeResult {
    /// The produced result, if any.
    ///
    /// If the node returns no result, it does NOT guarantee that the pipeline has terminated.
    /// It only means that more data has to be provided to the pipeline before a result can be produced.
    pub value: Option<QueryResult>,

    /// A boolean indicating if the pipeline should terminate after this result.
    ///
    /// If set, the pipeline should be terminated after yielding the item in [`PipelineNodeResult::value`], if any.
    pub terminated: bool,
}

impl PipelineNodeResult {
    /// Indicates that the pipeline has no result, but is not terminated. The pipeline requires more data to produce a result.
    pub const NO_RESULT: Self = Self {
        value: None,
        terminated: false,
    };

    pub const fn result(value: QueryResult, terminated: bool) -> Self {
        Self {
            value: Some(value),
            terminated,
        }
    }
}

/// Represents a slice of the query pipeline.
///
/// The pipeline is made up of all the nodes in the pipeline, with the final node being the item producer.
/// This struct represents some subset of the nodes, and the item producer.
///
/// This type exists so that nodes don't have to deal with slicing the list of nodes, and so that the item producer can be passed around easily.
/// Since the Item Producer and Pipeline Nodes are both owned by the [`QueryPipeline`](super::QueryPipeline), we can't create an owned type that contains both.
pub struct PipelineSlice<'a> {
    nodes: &'a mut [Box<dyn PipelineNode>],
    producer: &'a mut ItemProducer,
}

impl<'a> PipelineSlice<'a> {
    pub fn new(nodes: &'a mut [Box<dyn PipelineNode>], producer: &'a mut ItemProducer) -> Self {
        Self { nodes, producer }
    }

    /// Retrieves the next item from the first node in the span, passing the rest of the span as the "next" parameter.
    pub fn run(&mut self) -> crate::Result<PipelineNodeResult> {
        match self.nodes.split_first_mut() {
            Some((node, rest)) => {
                let result = node.next_item(PipelineSlice {
                    nodes: rest,
                    producer: self.producer,
                });
                tracing::debug!(node_name = ?node.name(), ?result, "completed pipeline node");
                result
            }
            None => {
                let result = self.producer.produce_item()?;
                Ok(result)
            }
        }
    }
}

/// Represents a node in the query pipeline.
///
/// Nodes are the building blocks of the query pipeline. They are used to represent different stages of query execution, such as filtering, sorting, and aggregation.
pub trait PipelineNode: Send + Debug {
    /// Retrieves the next item from this node in the pipeline.
    ///
    /// # Parameters
    /// * `next` - The next node in the pipeline, or `Ok(None)` if this is the last node in the pipeline.
    fn next_item(&mut self, rest: PipelineSlice) -> crate::Result<PipelineNodeResult>;

    /// Retrieves the name of this node, which defaults to it's type name.
    fn name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }
}

/// A pipeline node that limits the number of items that can pass through it by a fixed number.
///
/// This can be used to implement both `TOP` and `LIMIT` clauses in a query.
#[derive(Debug)]
pub struct LimitPipelineNode {
    /// The number of items that can pass through this node before it terminates the pipeline.
    remaining: u64,
}

impl LimitPipelineNode {
    pub fn new(limit: u64) -> Self {
        Self { remaining: limit }
    }
}

impl PipelineNode for LimitPipelineNode {
    fn next_item(&mut self, mut rest: PipelineSlice) -> crate::Result<PipelineNodeResult> {
        if self.remaining == 0 {
            tracing::debug!("limit reached, terminating pipeline");
            return Ok(PipelineNodeResult {
                value: None,
                terminated: true,
            });
        }

        match rest.run()? {
            PipelineNodeResult {
                value: Some(item),
                terminated,
            } => {
                tracing::debug!("limit not yet reached, returning item");
                self.remaining -= 1;
                Ok(PipelineNodeResult::result(
                    item,
                    terminated || self.remaining == 0,
                ))
            }

            // Pass through other results
            x => Ok(x),
        }
    }
}

/// A pipeline node that skips a fixed number of items before allowing any items to pass through it.
///
/// This can be used to implement both `OFFSET` clauses in a query.
#[derive(Debug)]
pub struct OffsetPipelineNode {
    /// The number of items that should be skipped before allowing any items to pass through.
    remaining: u64,
}

impl OffsetPipelineNode {
    pub fn new(offset: u64) -> Self {
        Self { remaining: offset }
    }
}

impl PipelineNode for OffsetPipelineNode {
    fn next_item(&mut self, mut rest: PipelineSlice) -> crate::Result<PipelineNodeResult> {
        while self.remaining > 0 {
            match rest.run()? {
                PipelineNodeResult { value: Some(_), .. } => {
                    tracing::debug!("offset not reached, skipping item");
                    self.remaining -= 1
                }

                // Pass through any early terminations or no results.
                x => return Ok(x),
            }
        }

        // Now, we're no longer skipping items, so we can pass through the rest of the pipeline.
        tracing::debug!("offset reached, returning item");
        rest.run()
    }
}

#[derive(Debug)]
pub struct AggregatePipelineNode {
    aggregators: Vec<Aggregator>,
    results: Option<VecDeque<Box<serde_json::value::RawValue>>>,
}

impl AggregatePipelineNode {
    pub fn from_names(names: Vec<String>) -> crate::Result<Self> {
        let mut aggregators = Vec::with_capacity(names.len());
        for name in names {
            aggregators.push(Aggregator::from_str(&name)?);
        }
        Ok(Self {
            aggregators,
            results: None,
        })
    }
}

impl PipelineNode for AggregatePipelineNode {
    fn next_item(&mut self, mut rest: PipelineSlice) -> crate::Result<PipelineNodeResult> {
        fn drain_result(
            results: &mut VecDeque<Box<serde_json::value::RawValue>>,
        ) -> crate::Result<PipelineNodeResult> {
            if let Some(value) = results.pop_front() {
                Ok(PipelineNodeResult::result(
                    QueryResult::RawPayload(value),
                    results.is_empty(),
                ))
            } else {
                Ok(PipelineNodeResult {
                    value: None,
                    terminated: true,
                })
            }
        }

        if let Some(results) = &mut self.results {
            return drain_result(results);
        }

        let result = rest.run()?;
        if let Some(item) = result.value {
            let aggregates = item.as_value_aggregates().ok_or_else(|| {
                ErrorKind::InvalidGatewayResponse
                    .with_message("expected single-partition aggregate results")
            })?;
            tracing::debug!(aggregator_count = self.aggregators.len(), "processing item");
            for clause_item in aggregates {
                for aggregator in &mut self.aggregators {
                    aggregator.aggregate(&clause_item)?
                }
            }
        }

        if result.terminated {
            tracing::debug!("aggregation complete, producing final result");
            let mut results = VecDeque::with_capacity(self.aggregators.len());
            for aggregator in self.aggregators.drain(..) {
                let value = aggregator.into_value()?;
                if let Some(value) = value {
                    let raw_value = serde_json::value::to_raw_value(&value).map_err(|e| {
                        ErrorKind::InternalError
                            .with_message(format!("failed to serialize aggregate result: {}", e))
                    })?;
                    results.push_back(raw_value);
                }
            }

            let result = drain_result(&mut results);
            self.results = Some(results);
            result
        } else {
            tracing::debug!("aggregation not yet complete, no result");
            Ok(PipelineNodeResult::NO_RESULT)
        }
    }
}
